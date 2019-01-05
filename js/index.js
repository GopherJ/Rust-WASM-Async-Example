Promise.resolve().then(async () => {
  const module = await import("../crate/pkg");
  await module.run();
  console.log("In JS");
});
