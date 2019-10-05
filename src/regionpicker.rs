use crate::steam::{EGCResults, HSteamPipe, IGameCoordinator, ISteamClient};

use crate::protos::cstrike15_gcmessages::{
    CMsgGCCStrike15_v2_MatchmakingClient2ServerPing, CMsgGCCStrike15_v2_MatchmakingStop,
    DataCenterPing, ECsgoGCMsg,
};
use protobuf::Message;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};
use winapi::{
    ctypes::c_void,
    um::{utilapiset::Beep, winnt::LONG},
};

#[derive(Debug)]
pub struct RegionPicker {
    pub original_send_message: usize,
    pub gc: &'static IGameCoordinator,
    pub datacenters: Arc<Mutex<HashMap<u32, Datacenter>>>,
    pub pipe: HSteamPipe,
    pub wndproc: LONG,
}

#[derive(Debug, Clone)]
pub struct Datacenter {
    pub desired_ping: i32,
    pub actual_ping: i32,
    pub name: String,
}

impl TryFrom<&DataCenterPing> for Datacenter {
    type Error = ();

    fn try_from(value: &DataCenterPing) -> Result<Self, Self::Error> {
        // Some datacenter pings may not have a ping (PerfectWorld?)
        if !value.has_data_center_id() || !value.has_ping() {
            return Err(());
        }

        let name = value.get_data_center_id();
        let ping = value.get_ping();

        //TODO: Find a cool way to do this with bitwise ops
        let mut string_name: [u8; 4] = name.to_le_bytes();
        // The name is encoded as u32
        string_name.swap(0, 2);

        // Some datacenters have multiple "sub-datacenters"
        // e.g. tyo and tyo1 for Tokyo and Tokyo1
        // if no variant exists it's a null byte.
        let length = match string_name[3] {
            0 => 3,
            _ => 4,
        };

        Ok(Datacenter {
            desired_ping: ping,
            actual_ping: ping,
            // This exists purely to debug what's happening from println!()
            name: String::from_utf8_lossy(&string_name[0..length]).to_string(),
        })
    }
}

impl Drop for RegionPicker {
    fn drop(&mut self) {
        self.disable();
        if let Some(client) = unsafe { ISteamClient::connect().as_mut() } {
            // This destroys our interface instance so all pointers are invalidated.
            client.BReleaseSteamPipe(self.pipe);
        }
    }
}

impl RegionPicker {
    pub fn send_message(&self, msg_type: u32, msg_data: *const c_void, msg_len: u32) -> EGCResults {
        type SendMessageFn = extern "fastcall" fn(
            ecx: *const IGameCoordinator,
            edx: *mut c_void,
            msg_type: u32,
            msg_data: *const c_void,
            msg_len: u32,
        ) -> EGCResults;

        let original: SendMessageFn = unsafe { std::mem::transmute(self.original_send_message) };
        original(
            self.gc as *const _ as *mut _,
            std::ptr::null_mut(),
            msg_type,
            msg_data,
            msg_len,
        )
    }

    pub fn apply_mode(&self, mode: char) {
        let mut data = self.datacenters.lock().unwrap();

        if data.is_empty() {
            return;
        }

        unsafe {
            Beep(650, 50);
        }

        match mode {
            '1' => {
                // Crap only
                for (_, entry) in data.iter_mut() {
                    if entry.actual_ping >= 200 {
                        entry.desired_ping = 15;
                    } else {
                        entry.desired_ping = 999;
                    }
                }
                msg!("Crap only mode activated");
            }
            '2' => {
                // Equilibrium
                for (_, entry) in data.iter_mut() {
                    entry.desired_ping = 15;
                }
                msg!("Equilibrium mode activated");
            }
            '3' => {
                // Weeb mode
                for (idx, entry) in data.iter_mut() {
                    // tyo and tyo1 -> Tokyo, Japan
                    if *idx == 0x0074_796F || *idx == 0x3174_796F {
                        entry.desired_ping = 15;
                    } else {
                        entry.desired_ping = 999;
                    }
                }
                msg!("Enabled weeb mode");
            }
            '4' => {
                // Reset
                for (_, entry) in data.iter_mut() {
                    entry.desired_ping = entry.actual_ping;
                }
                msg!("Spoofer reset");
            }
            _ => {}
        }
    }

    pub fn handle_message(&self, msg_type: ECsgoGCMsg, data: &[u8]) -> bool {
        if msg_type != ECsgoGCMsg::k_EMsgGCCStrike15_v2_MatchmakingClient2ServerPing {
            return false;
        }

        let mut message =
            protobuf::parse_from_bytes::<CMsgGCCStrike15_v2_MatchmakingClient2ServerPing>(data);
        if let Ok(msg) = &mut message {
            let mut data = self.datacenters.lock().unwrap();
            // We don't need to update the data
            if !data.is_empty() {
                // Clear the existing ones
                msg.clear_data_center_pings();

                // Apply the spoofed pings
                for (idx, entry) in data.iter() {
                    let mut new_ping = DataCenterPing::new();
                    new_ping.set_data_center_id(*idx);
                    new_ping.set_ping(entry.desired_ping);
                    msg.mut_data_center_pings().insert(0, new_ping);
                }

                // TODO: Extract this into a seperate function as it's duplicated
                if let Ok(mut out) = msg.write_to_bytes() {
                    let id: u32 = (ECsgoGCMsg::k_EMsgGCCStrike15_v2_MatchmakingClient2ServerPing
                        as u32)
                        | (1 << 31);
                    // Encode package type
                    ((u64::from(id)) << 32).to_le_bytes().iter().for_each(|i| {
                        out.insert(0, *i);
                    });
                    self.send_message(id, out.as_ptr() as *const c_void, out.len() as u32);
                }
                msg!("Applied spoofed pings!");
                return true;
            }

            // Store the "real" pings into our HashMap
            // TODO: Figure out if there's a way of getting this statically?
            msg.get_data_center_pings()
                .iter()
                .filter_map(|p| {
                    Datacenter::try_from(p)
                        .and_then(|d| Ok((p.get_data_center_id(), d)))
                        .ok()
                })
                .for_each(|(id, ping)| {
                    data.insert(id, ping);
                });

            // Send the MatchmakingStop packet to the GC
            let stop = CMsgGCCStrike15_v2_MatchmakingStop::default();

            if let Ok(mut out) = stop.write_to_bytes() {
                let id: u32 = (ECsgoGCMsg::k_EMsgGCCStrike15_v2_MatchmakingStop as u32) | (1 << 31);
                // Encode package type
                ((u64::from(id)) << 32).to_le_bytes().iter().for_each(|i| {
                    out.insert(0, *i);
                });
                // Stop queue
                self.send_message(id, out.as_ptr() as *const c_void, out.len() as u32);
            }

            msg!("Initialized datacenters. Please restart the queue.");
        }

        false
    }
}
