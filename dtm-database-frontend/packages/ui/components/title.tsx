import { css } from '@kuma-ui/core';

const title = css`
  color: blue;
`;

export function Title(): JSX.Element {
  return <h1 className={title}>わーい</h1>;
}
