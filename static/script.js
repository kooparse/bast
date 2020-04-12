(function () {
  // Let's begin.
  //
  // First, we want to execute our script only
  // when the initial document has been completely loaded
  // and parsed.
  const trackerUrl = window.__bast__trackerUrl;
  const website_id = window.__bast__website_id;
  // We're going to send only the location object and the referrer to
  // the server.
  const {href = "", origin = "", pathname = ""} = window.location;
  const {referrer = ""} = document;

  try {
    // We build the query.
    const query = generateQueryFromObject({
      href,
      origin,
      pathname,
      referrer
    });

    // Create img tag with built query.
    let ghost = document.createElement("img");
    ghost.setAttribute("alt", "analytics");
    // This will fetch the remote ressource and give us query values.
    ghost.src = `${trackerUrl}${query}`;

    // Cleanup.
    if (ghost.parentNode) {
      ghost.src = "";
      document.body.removeChild(ghost);
    }

    // The end.
    // clap clap clap.
  } catch (e) {
    // Finger crossed.
    console.error(e);
  }

  // Stolen, in consequence we should highly change this.
  function generateQueryFromObject(obj) {
    let keys = Object.keys(obj);

    return (
      "?" +
      keys
        .map(function (k) {
          return encodeURIComponent(k) + "=" + encodeURIComponent(obj[k]);
        })
        .join("&")
    );
  }
})();
