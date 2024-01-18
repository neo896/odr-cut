use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::collections::HashSet;
use std::io::Cursor;
use thiserror::Error;

pub fn xml_parse(xml_file: String, road_ids: HashSet<String>) {
    let mut buf = Vec::new();
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut reader = Reader::from_file(xml_file).unwrap();
    reader.trim_text(true);

    let mut inside_road = false;
    let mut keep_element = true;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"road" {
                    keep_element = e
                        .attributes()
                        .filter_map(Result::ok) // Safely ignore errors in attributes
                        .any(|attr| {
                            attr.key.as_ref() == b"id"
                                && road_ids.contains(std::str::from_utf8(&attr.value).unwrap_or(""))
                        });
                    inside_road = keep_element;
                }
                // Write out the event if it's not a filtered-out <road> element
                if keep_element {
                    writer.write_event(Event::Start(e.to_owned())).unwrap();
                }
            }
            Ok(Event::End(ref e)) => {
                // Write the end event if we're not inside a filtered-out <road>
                if keep_element {
                    writer.write_event(Event::End(e.to_owned())).unwrap();
                }
                // Reset flags when leaving a road element
                if e.name().as_ref() == b"road" {
                    inside_road = false;
                    keep_element = true;
                }
            }
            Ok(Event::Eof) => break, // End of file
            Ok(e) => {
                // Write all other events if we're not inside a filtered-out <road>
                if keep_element {
                    writer.write_event(e).unwrap();
                }
            }
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
        buf.clear(); // Clear the buffer for the next event
    }

    let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();
    println!("{}", result);
}
