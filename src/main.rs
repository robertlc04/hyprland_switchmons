use hyprland::{data::{Monitor, Monitors, Workspace, Workspaces}, dispatch::{Dispatch, DispatchType, WindowIdentifier}, shared::Address};
use hyprland::prelude::*;
use hyprland::Result;

#[derive(Debug)]
struct DB {
    active_mon: String,
    target_mon: String,
    address_win_active: Option<Address>,
    address_win_inactive: Option<Address>,
    name_active_wk: String,
    name_target_wk: String,
}

fn main() -> Result<()> {
    let mut x = DB::new()?;

    let _ = x.detect_wins();
    x.switch()?;

    println!("{:?}",x);

    Ok(())
}


impl DB {
    fn new() -> Result<Self> {
        let mons: Vec<Monitor> = Monitors::get()?.collect();

        let mut active_mon = String::new();
        let mut target_mon = String::new();
        let mut name_active_wk = String::new();
        let mut name_target_wk = String::new();

        for mon in mons {
            if mon.focused {
                active_mon = mon.name;
                name_active_wk = mon.active_workspace.name
            } else {
                target_mon = mon.name;
                name_target_wk = mon.active_workspace.name
            }
        }
        Ok(Self { active_mon, target_mon, address_win_active: None, address_win_inactive: None,name_active_wk ,name_target_wk })
    }
}

impl DB {
    pub fn detect_wins(&mut self) -> Result<()> {
        let aw = Workspaces::get()?.collect::<Vec<_>>()
            .into_iter()
            .filter(|x| x.name == self.name_target_wk || x.name == self.name_active_wk).collect::<Vec<_>>();
        for wk in aw {
            if wk.windows > 0 {
                if wk.name == self.name_active_wk 
                {
                    self.address_win_active = Some(wk.last_window.clone()) 
                }
                if wk.name == self.name_target_wk 
                { 
                    self.address_win_inactive = Some(wk.last_window.clone())  
                }
            }
        }
        Ok(())
    }
    pub fn switch(&self) -> Result<()> {
        if self.address_win_active.is_none() && self.address_win_inactive.is_some() {
            let _ = Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                    hyprland::dispatch::WorkspaceIdentifierWithSpecial::Name(&self.name_active_wk.to_string()), 
                    Some(WindowIdentifier::Address(self.address_win_inactive.clone().unwrap()))));
        } else if self.address_win_inactive.is_none() && self.address_win_active.is_some() {
            let _ = Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                    hyprland::dispatch::WorkspaceIdentifierWithSpecial::Name(&self.name_target_wk.to_string()), 
                    Some(WindowIdentifier::Address(self.address_win_active.clone().unwrap()))));
        } else {
             let _ = Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                    hyprland::dispatch::WorkspaceIdentifierWithSpecial::Name(&self.name_target_wk.to_string()), 
                    Some(WindowIdentifier::Address(self.address_win_active.clone().unwrap()))));
            let _ = Dispatch::call(DispatchType::MoveToWorkspaceSilent(
                    hyprland::dispatch::WorkspaceIdentifierWithSpecial::Name(&self.name_active_wk.to_string()), 
                    Some(WindowIdentifier::Address(self.address_win_inactive.clone().unwrap()))));
        }
        Ok(())
    }
}

// impl DB {
//     fn load_data() -> Result<()> {
//         let mons: Vec<_> = Monitors::get()?.collect();
//         Ok(())
//     }
// }

// fn active_workspaces() -> Result<Vec<WindowsChange>> {
//     let aw = match Workspace::get_active() {
//         Ok(o) => o,
//         Err(e) => panic!("Error: {}", e),
//     };
//
//     let mons: Vec<Monitor> = Monitors::get()?.collect();
//
//     let second_mon: Vec<Monitor> = Monitors::get()?
//         .into_iter()
//         .filter(|x| x.name != aw.monitor)
//         .collect();
//
//     let mut change: Vec<WindowsChange> = Vec::new();
//
//     change.push(WindowsChange {
//             origin: aw.id,
//             address: aw.last_window.clone(),
//         });
//
//     let held: Vec<_> = Workspaces::get()?.into_iter().by_ref().collect();
//
//     let tmp: Vec<Workspace> = held
//         .clone()
//         .into_iter()
//         .filter(|x| x.id == second_mon[0].active_workspace.id && x.monitor != aw.monitor)
//         .collect();
//
//     change.push(WindowsChange {
//         origin: second_mon[0].active_workspace.id,
//         address: tmp[0].last_window.clone(),
//     });
//
//     println!("MONS: {:?}\nActive Wk: {:?}\nHeld: {:?}",mons,aw,held);
//
//     Ok(change)
// }
