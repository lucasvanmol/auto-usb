fn main() {
    windows::build! {
        Windows::Win32::Storage::FileSystem::GetLogicalDrives,
    };
}