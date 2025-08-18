use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct Shell {
    shell: String,
    command: Option<Vec<String>>,
    termux_fs: PathBuf,
}

impl Shell {
    pub fn new(shell: String, command: Option<Vec<String>>) -> Self {
        const TERMUX_FS: &str = "/data/data/com.termux/files";
        Shell {
            shell: shell,
            command: command,
            termux_fs: PathBuf::from(TERMUX_FS),
        }
    }

    pub fn new_shell(&self) -> Command {
        let su_file = self.termux_fs.join("usr/bin/su");
        let busybox = self.get_busybox();
        let env_vars = self.gen_env_vars();
        let shell_arg = &format!("{} env -i {} {}", busybox.display(), env_vars, self.shell);

        let mut shell = Command::new(&su_file);
        shell.args(["-i", "-c"]);
        shell
            .arg(self.append_command(shell_arg))
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit());
        shell
    }

    fn append_command(&self, shell_arg: &str) -> String {
        let command = &self.command;
        match command {
            Some(command) => format!("{} -c \"{}\"", shell_arg, command.join(" ")),
            None => shell_arg.to_string(),
        }
    }

    fn get_busybox(&self) -> PathBuf {
        let toybox = PathBuf::from("/system/bin/toybox");
        let busybox = PathBuf::from("/system/bin/busybox");
        let fallback = PathBuf::from("toybox"); // search in path
        if toybox.exists() {
            toybox
        } else if busybox.exists() {
            busybox
        } else {
            fallback
        }
    }

    fn gen_env_vars(&self) -> String {
        const ANDROID_BIN_PATH: &str = "/system/bin";
        let home = self.termux_fs.join("root");
        let prefix = self.termux_fs.join("usr");
        let termux_bin_path = self.termux_fs.join("usr/bin");
        format!(
            "HOME={} PATH={}:{} TERMUX_FS={} PREFIX={}",
            home.display(),
            termux_bin_path.display(),
            ANDROID_BIN_PATH,
            self.termux_fs.display(),
            prefix.display()
        )
    }
}
