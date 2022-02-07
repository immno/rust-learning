use std::{env, fmt};
use std::cmp::max;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if is_total_warehouse(&args) {
        let hdfs_info: HdfsInfo = Default::default();
        sleep(Duration::from_millis(3000));
        hdfs_info.print_stdout();
        return;
    }
    let child = Command::new(HDFS_COMMAND)
        .args(args)
        .spawn();

    // 优雅地处理非正常输入
    match child {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(e) => eprintln!("{}", e),
    };
}

// 判断是否需要拦截
fn is_total_warehouse(args: &Vec<String>) -> bool {
    args.iter().any(|x| x.eq(WAREHOUSE_PATH1) || x.eq(WAREHOUSE_PATH2))
}

const SPACE: usize = 2;
const HDFS_COMMAND: &str = "/etc/alternatives/hdfs";
const WAREHOUSE_PATH1: &str = "/user/hive/warehouse";
const WAREHOUSE_PATH2: &str = "/user/hive/warehouse/";

struct HdfsInfo {
    size: Info,
    disk_space: Info,
    path_name: Info,
}

struct Info {
    name: &'static str,
    value: &'static str,
}

impl Info {
    fn width(&self) -> usize {
        max(self.name.len(), self.value.len()) + SPACE
    }
}

impl HdfsInfo {
    pub fn print_stdout(&self) {
        println!("{}", self);
    }
}

impl Default for HdfsInfo {
    fn default() -> Self {
        Self {
            size: Info { name: "SIZE", value: "105.2 T" },
            disk_space: Info { name: "DISK_SPACE_CONSUMED_WITH_ALL_REPLICAS", value: "315.6 T" },
            path_name: Info { name: "FULL_PATH_NAME", value: WAREHOUSE_PATH1 },
        }
    }
}

impl fmt::Display for HdfsInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "{:size_width$}{:disk_space_width$}{:path_name_width$}\n{:size_width$}{:disk_space_width$}{:path_name_width$}",
               self.size.name, self.disk_space.name, self.path_name.name, self.size.value, self.disk_space.value, self.path_name.value,
               size_width = self.size.width(), disk_space_width = self.disk_space.width(), path_name_width = self.path_name.width()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_warehouse() {
        let str: Vec<String> = "hdfs dfs -du -s -h -v /user/hive/warehouse/business_z4zp.db/"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        println!("{:?}", str);
        assert_eq!(is_total_warehouse(&str), false);
        let str: Vec<String> = "hdfs dfs -du -s -h -v /user/hive/warehouse/"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        println!("{:?}", str);
        assert_eq!(is_total_warehouse(&str), true);
    }

    #[test]
    fn test_table() {
        let hdfs_info: HdfsInfo = Default::default();
        hdfs_info.print_stdout();
    }
}