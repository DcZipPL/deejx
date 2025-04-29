// todo: There will be platform code required. midir supports unix, but windows support was removed. For windows `rtmidi` bindings will be used or loopMIDI virtual device

/*use midir::{MidiOutput, MidiOutputConnection};
fn debug_midi(value: u8, cc_number: u8) -> anyhow::Result<()> {
    let midi_out = MidiOutput::new("Deejx Midi Output")?;
    let out_ports = midi_out.ports();

    for (i, p) in out_ports.iter().enumerate() {
        println!("{}: {}", i, midi_out.port_name(p)?);
    }

    // Suppose loopMIDI created a port named "loopMIDI Port"
    // You find it and connect to it
    let port = &out_ports[0]; // pick based on the list
    let mut conn_out = midi_out.connect(port, "loopMIDI Port").unwrap();
    conn_out.send(&[0xB0, 1, 64])?; // send a CC message
    Ok(())
}*/