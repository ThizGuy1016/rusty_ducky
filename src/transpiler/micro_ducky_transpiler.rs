// transpiles the parser tokens to a micropython file to be interpreted on the microcontroller
use crate::DuckyError;
use std::fs;
use std::io::Write;

use super::{RELEASE, Args, KeyReport};

const TEMPLATE: &str = "import usb_hid, time
def report(report_code, rel = True, slep = 0.02) -> None:
    kbd.send_report(bytearray(report_code), 1)
    time.sleep(slp)
    if rel:
        kbd.send_report(bytearray([0]*8))
        time.sleep(0.001)
slp = 0
kbd = [device for device in usb_hid.devices if device.usage_page == 0x1 and device.usage == 0x6 and hasattr(device, 'send_report')][0]
";

fn read_template(args: &Args) -> Result<String, DuckyError> { 
    let filename = args.template_file.as_ref().unwrap();
    match fs::read_to_string(filename)  {
        Ok(f) => Ok(f),
        Err(e) => {
            let verbose_info = format!("{:?}", e);
            let err_data = format!("{} was not found in current directory.", filename);
            Err(DuckyError::new("Unable to read provided template file.", Some(err_data.as_str()), (verbose_info.as_str(), args.verbose)))
        }
    }
}

fn write_microducky(filename: &String, content: &String) -> Result<(), DuckyError> {
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn transpile(args: &Args, report_buf: &Vec<KeyReport>) -> Result<(), DuckyError> {
    let mut file_buf: String;
    if args.template_file == None { file_buf = TEMPLATE.to_string() }
    else { file_buf = read_template(&args)?}
    let mut str_report: String;
    for (i, report) in report_buf.iter().enumerate() {
        
        if report == &RELEASE { continue }
        if report[0] == 100 && report[2] == 0_u16 { str_report = format!("time.sleep(slp)\n")}
        else if report[0] == 100 { str_report = format!("time.sleep({})\n", (report[2] as f32) / 1000_f32) }
        else if report[0] == 200 { str_report = format!("slp = {}\n", (report[2] as f32) / 1000_f32) }
        else { 
            if i != report_buf.len() - 1 && report_buf[i + 1] == RELEASE { str_report = format!("report({:?})\n", report) }
            else { str_report = format!("report({:?}, rel=False)\n", report) }
         }
        file_buf.push_str(str_report.as_str());
    }

    write_microducky(&args.output_file, &file_buf)?;
    Ok(())
}