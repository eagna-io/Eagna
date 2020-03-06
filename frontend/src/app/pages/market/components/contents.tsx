import React from "react";
import styled from "styled-components";

const Contents: React.FC = () => {
  return (
    <Container>
    	this is Contents
    </Container>
  );
};

export default Contents;

const Container = styled.div`
  position: relative;
  background-color: #242423;
  padding: 20px;
  display: flex;
  justify-content: space-between;
	align-items: center;
	color: white;
`;
