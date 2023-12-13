import "../plugin/constant";

function main() {
  const myLinkedin = () => {
    console.log(constant.str("app.PROFILE.LINKEDIN"));
  };

  const myGithub = () => {
    console.log(constant.str("app.PROFILE.GH"));
  };

  function say() {
    console.log(constant.str("app.MESSAGE"));
    console.log("ok? " + constant.str("app.OK"));
  }

  function year() {
    console.log("year: " + constant.number("app.YEAR"));
  }

  say();
  myLinkedin();
  myGithub();
}

main();
