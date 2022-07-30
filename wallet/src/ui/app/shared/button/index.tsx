// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import cl from 'classnames';
import { memo } from 'react';
import { Link } from 'react-router-dom';

import type {
    ReactNode,
    ButtonHTMLAttributes,
    AnchorHTMLAttributes,
} from 'react';

import st from './Button.module.scss';

export type ButtonProps = {
    className?: string;
    mode?: 'neutral' | 'primary';
    size?: 'small' | 'large';
    children: ReactNode | ReactNode[];
    disabled?: boolean;
    title?: string;
} & (
    | {
          href: string;
          onClick?: AnchorHTMLAttributes<HTMLAnchorElement>['onClick'];
      }
    | {
          onClick?: ButtonHTMLAttributes<HTMLButtonElement>['onClick'];
      }
);

function Button(props: ButtonProps) {
    const {
        className,
        mode = 'neutral',
        size = 'large',
        children,
        disabled = false,
        title,
    } = props;
    const commonProps = {
        className: cl(st.container, className, st[mode], st[size], {
            [st.disabled]: disabled,
        }),
        disabled,
        children,
        title,
    };
    if ('href' in props) {
        return (
            <Link to={props.href} {...commonProps} onClick={props.onClick} />
        );
    }
    return <button type="button" {...commonProps} onClick={props.onClick} />;
}

export default memo(Button);
