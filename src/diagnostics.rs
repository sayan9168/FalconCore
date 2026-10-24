// Rich Terminal Error Reporting extension for FalconCore diagnostics
pub fn report_advanced_error(file: &str, line: usize, col: usize, message: &str) {
    eprintln!("\x1b[1;31merror[FalconE001]:\x1b[0m {} \n  --> {}:{}:{}", message, file, line, col);
    eprintln!("   \x1b[1;34m|\x1b[0m");
    eprintln!("{:3} \x1b[1;34m|\x1b[0m    ^ [Context verification or type mismatch detected]", line);
    eprintln!("   \x1b[1;34m|\x1b[0m");
}
