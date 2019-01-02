Promise.resolve().then(async () => {
  const module = await import("../crate/pkg");
  console.log("begin");
  await module.run();
  console.log("end");
});
