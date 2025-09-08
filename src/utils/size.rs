const KB: usize = 1024;
const MB: usize = KB * 1024;
const GB: usize = MB * 1024;

pub fn byte_size_to_string(bytes: usize) -> String {
  if bytes >= GB {
    return format!("{:.1} GB", bytes / GB);
  }
  if bytes >= MB {
    return format!("{:.1} MB", bytes / MB);
  }
  if bytes >= KB {
    return format!("{:.1} KB", bytes / KB);
  }

  return format!("{} B", bytes);
}
