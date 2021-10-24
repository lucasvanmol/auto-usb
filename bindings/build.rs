fn main() {
    windows::build! {
        Windows::Win32::Foundation::*,
        Windows::Win32::Graphics::Gdi::*,
        Windows::Win32::Storage::FileSystem::GetLogicalDrives,
        Windows::Win32::System::SystemServices::CHAR
    };
}