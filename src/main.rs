mod handle_xml;
mod rm;

use handle_xml::xml_parse;
use rm::cal_road_id;

fn main() {
    let odr_path =
        r"/home/adrdc/Downloads/esmini-demo_Linux/esmini-demo/resources/xodr/soderleden.xodr"
            .to_string();
    let position_path = r"/home/adrdc/code/rust/odr_cutting/src/example/position.txt".to_string();
    match cal_road_id(position_path, odr_path) {
        Ok(road_id) => xml_parse(
            r"/home/adrdc/Downloads/esmini-demo_Linux/esmini-demo/resources/xodr/soderleden.xodr"
                .to_string(),
            road_id,
        ),
        Err(e) => println!("{:?}", e),
    }
}
