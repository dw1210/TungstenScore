use crate::checks::*;
use crate::config::*;

pub fn run_scan()-> ScanSummary{
    let uac_result = uac::run();
    print_result(&uac_result);
    let firewall_result = firewall::run();
    print_result(&firewall_result);
    let results = vec![
        uac_result,
        firewall_result,
    ];

    let total_max_score: u32 = results.iter().map(|result| result.score).sum();
    let total_score: u32 = results.iter().map(|result| result.score).sum();

    ScanSummary{
        total_score,
        total_max_score,
    }
}

fn print_result(check_result: &CheckResult){
    let score: String = if check_result.max_score == 0{
        String::from("?")
    }else{
        format!("+{}",check_result.score).to_string()
    };
    println!("{}: {} [{}]",check_result.name, check_result.status.display(), score);
}

pub fn calculate_final_score(scan_summary: ScanSummary) -> f64{
    let total_score = scan_summary.total_score;
    let total_max_score = scan_summary.total_max_score;
    if total_max_score != 0 {
        (total_score as f64 / total_max_score as f64) * 100_f64
    }else {
        0.0
    }
}

pub fn print_final_score(score: f64){
    println!(r#"
--Scan Score--
> {score:.2}/100.00
    "#);
}

pub fn perform_scan_and_print(){
    print_final_score(calculate_final_score(run_scan()));
}