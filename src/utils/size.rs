const KB: f32 = 1024.0;
const MB: f32 = KB * 1024.0;
const GB: f32 = MB * 1024.0;

pub fn byte_size_to_string(bytes: usize) -> String {
  let bytes = bytes as f32;
  if bytes >= GB {
    return format!("{:.1} GB", bytes as f32 / GB);
  }
  if bytes >= MB {
    return format!("{:.1} MB", bytes / MB);
  }
  if bytes >= KB {
    return format!("{:.1} KB", bytes / KB);
  }

  return format!("{} B", bytes);
}

pub fn gigabytes(gb: usize) -> usize {
  return gb * GB as usize;
}
