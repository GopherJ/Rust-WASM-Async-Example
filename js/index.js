Promise.resolve().then(async () => {
  const module = await import("../crate/pkg");
  module.run();

  console.log("begin");
  await module.sleep(1000);
  console.log("end");
});
