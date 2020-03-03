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
  const [timer, setTimer] = React.useState<number | undefined>();
  const [isActive, setIsActive] = React.useState(false);

  return (
    <StyledButton
      className={className}
      unselectable="on"
      onPointerDown={(e) => {
        e.stopPropagation();
        const timer = setTimeout(() => {
          setIsActive(true);
        }, threshold);
        setTimer(timer);
      }}
      onPointerUp={(e) => {
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
