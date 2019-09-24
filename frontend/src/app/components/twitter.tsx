import React from "react";
import styled from "styled-components";

interface Props {
  center?: boolean;
}

export default ({ center }: Props) => {
  React.useEffect(() => {
    const script = document.createElement("script");
    script.src = "https://platform.twitter.com/widgets.js";
    script.charset = "utf-8";
    script.async = true;
    document.body.appendChild(script);
  }, []);

  return (
    <Container center={center}>
      <a
        className="twitter-timeline"
        data-theme="light"
        data-height="500"
        href="https://twitter.com/eagna_io?ref_src=twsrc%5Etfw"
      >
        Tweets from Eagna
      </a>
    </Container>
  );
};

const Container = styled("div")<{ center?: boolean }>`
  width: 90%;
  max-width: 500px;
  ${props => (props.center ? "margin: 20px auto" : "margin: 20px 0")};
`;
