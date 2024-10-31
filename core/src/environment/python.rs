pub fn check_python(current_python: String) -> bool {
    // 注意！ Windows 10/11即使没有安装，也不会报错（因为微软不知道怎么想得，如果未安装居然会默认打开应用商店...）
    std::process::Command::new(current_python).arg("--version").output().is_ok()
}
