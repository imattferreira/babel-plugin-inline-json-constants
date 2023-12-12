import "../plugin/constant";

function main() {
  const myLinkedin = () => {
    console.log(constant("app.PROFILE.LINKEDIN"));
  };

  const myGithub = () => {
    console.log(constant("app.PROFILE.GH"));
  };

  function say() {
    console.log(constant("app.MESSAGE"));
  }

  say();
  myLinkedin();
  myGithub();
}

main();
