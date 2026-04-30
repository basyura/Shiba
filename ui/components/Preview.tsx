import * as React from 'react';
import { useContext } from 'react';
import Box from '@mui/material/Box';
import Divider from '@mui/material/Divider';
import { Resizable } from 're-resizable';
import { WindowBar } from './WindowBar';
import { SideBar } from './SideBar';
import { Article } from './Article';
import { ConfigContext } from './ConfigContext';
import { colorScheme } from '../css';
import type { MarkdownReactTree } from '../markdown';
import type { Dispatch, Heading } from '../reducer';

const NAV_RESIZE_DIRECTION = {
    top: false,
    right: false,
    bottom: false,
    left: true,
    topRight: false,
    bottomRight: false,
    bottomLeft: false,
    topLeft: false,
};

const NAV_DEFAULT_SIZE = {
    width: '20%',
    height: '100%',
};

export interface Props {
    tree: MarkdownReactTree;
    headings: Heading[];
    path: string | null;
    dispatch: Dispatch;
}

export const Preview: React.FC<Props> = ({ tree, headings, path, dispatch }) => {
    const { titleBar, vibrant, borderTop } = useContext(ConfigContext);

    if (tree.root === null) {
        return <></>;
    }

    // Note: `SxProps` type is useless here
    const sx: {
        bgcolor?: string;
        borderTop?: number;
        borderColor?: string;
        boxSizing?: string;
    } = {};
    if (!vibrant) {
        sx.bgcolor = colorScheme.isDark ? 'grey.900' : 'grey.100';
    }
    if (borderTop) {
        sx.borderTop = 1;
        sx.borderColor = 'divider';
        sx.boxSizing = 'border-box';
    }

    return (
        <Box component="main" sx={sx}>
            <Article tree={tree} dispatch={dispatch} currentPath={path} key={path ?? 'no-path'} />
            <Divider orientation="vertical" />
            <Resizable defaultSize={NAV_DEFAULT_SIZE} minWidth="200px" enable={NAV_RESIZE_DIRECTION} as="nav">
                {titleBar && <WindowBar />}
                <SideBar headings={headings} path={path} />
            </Resizable>
        </Box>
    );
};
