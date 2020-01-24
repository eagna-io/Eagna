import styled from "styled-components";

export const MinPcWidth = 980;
export const MaxTabletWidth = MinPcWidth - 1;
export const MinTabletWidth = 768;
export const MaxMobileWidth = MinTabletWidth - 1;

export function pc(css: string): string {
  return `@media (min-width: ${MinPcWidth}px) {
    ${css}
  }`;
}

export function tablet(css: string): string {
  return `@media (min-width: ${MinTabletWidth}px) and (max-width: ${MaxTabletWidth}px) {
    ${css}
  }`;
}

export function mobile(css: string): string {
  return `@media (max-width: ${MaxMobileWidth}px) {
    ${css}
  }`;
}

export const Container = styled.div`
  width: 100vw;
  padding-left: 20px;
  padding-right: 20px;

  ${tablet(`
    padding-left: calc((100vw - ${MinTabletWidth}px) / 2 + 20px);
    padding-right: calc((100vw - ${MinTabletWidth}px) / 2 + 20px);
  `)}

  ${pc(`
    padding-left: calc((100vw - ${MinPcWidth}px) / 2 + 20px);
    padding-right: calc((100vw - ${MinPcWidth}px) / 2 + 20px);
  `)}
`;

export const Mobile = styled.div`
  display: none;
  ${mobile(`
    display: block;
  `)}
`;

export const Tablet = styled.div`
  display: none;
  ${tablet(`
    display: block;
  `)}
`;

export const Pc = styled.div`
  display: none;
  ${pc(`
    display: block;
  `)}
`;
