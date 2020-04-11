import React from "react";
import styled from "styled-components";

import { WhiteBaseColor } from "app/components/color";

interface Props {
  numer: number;
  denom: number;
}

export const Score: React.FC<Props> = ({ numer, denom }) => {
  return (
    <Container>
      正解数
      <UserScore>
        {numer}<Slash>/</Slash>{denom}<Every>問中</Every>
        {/* {numer}/{denom}問中 */}
      </UserScore>
    </Container>
  )
};

const Container = styled.div`
  font-size: 8px;
  text-align: center;
  color: ${WhiteBaseColor.hex};
`;

const UserScore = styled.div`
  font-size: 20px;
  letter-spacing: 0.71px;
  font-weight: 800;
  display: flex;
  align-items: center;
`;

const Slash = styled.span`
  display: inline-block;
  font-size: 10px;
  font-weight: 600;
  margin: 0px 2px;
`;

const Every = styled.span`
  display: inline-block;
  font-size: 10px;
  font-weight: 800;
  margin-left: 3px;
  margin-top: 6px;
`;
