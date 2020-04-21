import React, { ReactElement } from "react";
import config from "next/config";
import { useClipboard, Code, Button } from "@chakra-ui/core";

const { API_ENDPOINT, SCRIPT_ENDPOINT } = config().publicRuntimeConfig;

const getSnippet = (websiteId: string | number): string => {
  const { origin } = window.location;
  return `<script>
  (function() {
    window.__bast__website_id = ${websiteId};
    window.__bast__trackerUrl = "${origin}${API_ENDPOINT}/ghost.png";

    var doNotTrack = navigator.doNotTrack 
      && navigator.doNotTrack === "1" || navigator.doNotTrack === "yes"

    if (doNotTrack) {
      return;
    }

    var script = document.createElement('script');
    script.src = "${origin}${SCRIPT_ENDPOINT}";
    script.async = false;
    document.head.appendChild(script);
  })();
</script>
`;
};

const CodeSnippet = ({ website }: { website: Website }): ReactElement => {
  const snippet = getSnippet(website.id);
  const { onCopy, hasCopied } = useClipboard(snippet);

  return (
    <Code
      my="2"
      borderRadius="md"
      p="4"
      position="relative"
      width="100%"
      style={{ whiteSpace: "pre-wrap" }}
    >
      <Button
        onClick={onCopy}
        size="sm"
        variantColor="teal"
        position="absolute"
        right="5"
      >
        {hasCopied ? "Copied" : "Copy"}
      </Button>
      {snippet}
    </Code>
  );
};

export default CodeSnippet;
