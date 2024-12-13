use crate::read_file::read_input_file;

fn is_report_safe(report: &Vec<u64>) -> bool {
    let diff_list = report
        .windows(2)
        .map(|elem_pair| elem_pair[0] as i64 - elem_pair[1] as i64);

    let all_decreasing = diff_list.clone().all(|diff| diff > 0);
    let all_increasing = diff_list.clone().all(|diff| diff < 0);
    let all_diff_one_or_two = diff_list
        .clone()
        .all(|diff| diff.abs_diff(0) == 1 || diff.abs_diff(0) == 2 || diff.abs() == 3);

    return all_diff_one_or_two && (all_increasing || all_decreasing);
}

fn is_report_mostly_safe(report: &Vec<u64>) -> bool {
    for remove_idx in 0..report.len() {
        let report_without_element: Vec<u64> = report
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| if idx != remove_idx { Some(val) } else { None })
            .collect();

        let trimmed_report_safe = is_report_safe(&report_without_element);

        if trimmed_report_safe{
            return true;
        }
    }

    return false;
}


pub fn p1() {
    let file_src = "inputs/day2.txt";
    let read_file = read_input_file(file_src);
    let report_vec = read_file.expect("unable to read or parse file");
    println!("{}", report_vec.len());

    let safe_report_count = report_vec
        .iter()
        .filter(|report| is_report_safe(report))
        .count();

    println!("{}", safe_report_count);
}

pub fn p2() {
    let file_src = "inputs/day2.txt";
    let report_vec = read_input_file(file_src).expect("unable to read or parse file");
    println!("{}", report_vec.len());

    let safe_report_count = report_vec
        .iter()
        .filter(|report| is_report_mostly_safe(report))
        .count();

    println!("{}", safe_report_count);
}
