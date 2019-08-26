use std::time::{
  Duration,
  Instant,
};

pub fn create_duration_from_sec(seconds: f32) -> Duration
{
   Duration::new(seconds.trunc() as u64, (seconds.fract() * f32::powi(10.0, 9)) as u32)
}

pub fn create_secf_from_duration(duration: Duration) -> f32
{
  //duration.as_nanos()
  (duration.as_secs() as f32)
    + ((duration.subsec_millis() as f32) * f32::powi(10.0, -3))
    //+ ((duration.subsec_millis() as f32) * f32::powi(10.0, -6))
    //+ ((duration.subsec_nanos()() as f32) * f32::powi(10.0, -9))
}

pub fn get_progress_into_duration_normalized(progress: Instant, begin: Instant, duration: Duration) -> f32
{
  let elapsed_time: Duration = begin.duration_since(progress);

  match duration.checked_sub(elapsed_time)
  {
    Some(time_delta) =>
    {
      create_secf_from_duration(time_delta)
    },
    None => 1.0
  }
}
