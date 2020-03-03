import React from "react";
import styled from "styled-components";

interface Props {
  onPress: () => void;
  threshold: number;
  className?: string;
}

export const PressButton: React.FC<Props> = ({
  onPress,
  threshold,
  className,
  children
}) => {
  const [timer, setTimer] = React.useState();
  const [isActive, setIsActive] = React.useState(false);

  return (
    <StyledButton
      className={className}
      unselectable="on"
      onTouchStart={(e) => {
        e.preventDefault();
        e.stopPropagation();
        const timer = setTimeout(() => {
          setIsActive(true);
        }, threshold);
        setTimer(timer);
      }}
      onTouchEnd={(e) => {
        e.preventDefault();
        e.stopPropagation();
        clearTimeout(timer);
        if (isActive) {
          onPress();
        }
        setIsActive(false);
      }}
    >
      {children}
    </StyledButton>
  );
};

const StyledButton = styled.button`
  user-select: none;
  -webkit-touch-callout: none;
`;
