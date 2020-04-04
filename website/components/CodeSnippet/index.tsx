import React, { ReactElement } from "react";
import config from "next/config";
import { Code } from "@chakra-ui/core";

const { API_URL, SCRIPT_URL } = config().publicRuntimeConfig;

const CodeSnippet = ({
  user,
  website
}: {
  user: User;
  website: Website;
}): ReactElement => {
  return (
    <Code my="2" borderRadius="md" p="4">
      {`
          <script>
            (function() {
              window.__bast__website_id = ${website.id};
              window.__bast__user_id = ${user.id};
              window.__bast__trackerUrl = "${API_URL}/ghost.png";

              var script = document.createElement('script');
              script.src = "${SCRIPT_URL}";
              script.async = false;
              document.head.appendChild(script);
            })();
          </script>
        `}
    </Code>
  );
};

export default CodeSnippet;
