export const foo = () => {
    console.log("foo");
};

export const _sleep =
    timeout => new Promise(resolve => setTimeout(resolve, timeout));
