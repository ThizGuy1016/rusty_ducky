// transpiles the parser tokens to a micropython file to be interpreted on the microcontroller
use crate::DuckyError;

use super::{ARGS, RELEASE, KeyReport, ducky_io::*};

const TEMPLATE: &str = "import usb_hid, time
def report(report_code, rel = True, slep = 0.02) -> None:
    kbd.send_report(bytearray(report_code), 1)
    time.sleep(slep)
    if rel:
        kbd.send_report(bytearray([0]*8))
        time.sleep(0.02)
slp = 0
kbd = [device for device in usb_hid.devices if device.usage_page == 0x1 and device.usage == 0x6 and hasattr(device, 'send_report')][0]
time.sleep(1)
";
    

pub fn transpile(payload_tokens: Vec<KeyReport>) -> Result<(), DuckyError> {
    let mut file_buf = read_template()?;
    let mut str_report: String;
        
    for (i, report) in payload_tokens.iter().enumerate() {
            
        if report == &RELEASE { continue }
        // empty DELAY

        if report[1] == 100 && report[2] == 0_u16 { str_report = format!("time.sleep(slp)\n")}
        // DELAY
        else if report[1] == 100 { str_report = format!("time.sleep({})\n", (report[2] as f32) / 1000_f32) }
        // DEFAULT_DELAY
        else if report[1] == 200 { str_report = format!("slp = {}\n", (report[2] as f32) / 1000_f32) }
        // not a special command

        else { 
            // if this is not the last report and the next report is release
            if i != payload_tokens.len() - 1 && payload_tokens[i + 1] == RELEASE { str_report = format!("report({:?})\n", report) }
            // we tell the python script to not release
            else { str_report = format!("report({:?}, rel=False)\n", report) }
        }
        file_buf.push_str(str_report.as_str());
    }
    
        ducky_write_file(&ARGS.output_file, &file_buf)?;

        Ok(())
    }

fn read_template() -> Result<String, DuckyError> { 
    match &ARGS.template_file {
        Some(f) => Ok(ducky_read_file(f)?),
        None => Ok(TEMPLATE.to_string())
    }
}