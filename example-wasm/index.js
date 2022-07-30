const rust = import('./pkg');

rust
    .then(console.log)
    .catch(console.error);