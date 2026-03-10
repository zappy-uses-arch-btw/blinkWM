use std::sync::Arc;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{ConnectionExt as _, WindowClass, CreateWindowAux, EventMask};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        _NET_SYSTEM_TRAY_OPCODE,
        _NET_SYSTEM_TRAY_S0,
        MANAGER,
    }
}

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting blinkwm-tray...");

    let (conn, screen_num) = x11rb::connect(None)?;
    let conn = Arc::new(conn);
    let screen = &conn.setup().roots[screen_num];
    let root = screen.root;
    let atoms = Atoms::new(conn.as_ref())?.reply()?;

    // 1. Create Tray Window (Invisible, just for ownership)
    let tray_win = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        tray_win,
        root,
        -1, -1, 1, 1,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::default()
            .event_mask(EventMask::STRUCTURE_NOTIFY)
    )?;

    // 2. Set Selection Owner for the Tray
    conn.set_selection_owner(tray_win, atoms._NET_SYSTEM_TRAY_S0, Time::CURRENT_TIME)?;

    // 3. Send Manager ClientMessage
    let mut data = [0u32; 5];
    data[0] = Time::CURRENT_TIME.into();
    data[1] = atoms._NET_SYSTEM_TRAY_S0;
    data[2] = tray_win;
    
    let event = ClientMessageEvent {
        response_type: CLIENT_MESSAGE_EVENT,
        format: 32,
        sequence: 0,
        window: root,
        type_: atoms.MANAGER,
        data: ClientMessageData::from(data),
    };
    conn.send_event(false, root, EventMask::STRUCTURE_NOTIFY, event)?;

    conn.flush()?;
    info!("System Tray ownership acquired.");

    // Simple event loop
    loop {
        let _event = conn.wait_for_event()?;
    }
}

use x11rb::protocol::xproto::{ClientMessageEvent, ClientMessageData, CLIENT_MESSAGE_EVENT, Time};
