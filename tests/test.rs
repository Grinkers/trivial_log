use log::{info, LevelFilter};

#[test]
fn test_for_unit() {
  let receiver = trivial_log::init_for_unit_test(LevelFilter::Info).expect("Failed to init logger");
  info!("Bomboclat");

  trivial_log::free();

  let mut messages: Vec<String> = Vec::new();
  while let Ok(received) = receiver.recv() {
    messages.push(received);
  }

  assert_eq!(messages.len(), 1);
  assert_eq!(messages[0].as_str(), "[I] Bomboclat\n");
}
