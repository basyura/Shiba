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
    width: '150px',
    height: '100%',
};
const NAV_HIDDEN_STYLE = {
    display: 'none',
};

export interface Props {
    tree: MarkdownReactTree;
    headings: Heading[];
    path: string | null;
    dispatch: Dispatch;
    sideBar: boolean;
    alwaysOnTop: boolean;
}

export const Preview: React.FC<Props> = ({ tree, headings, path, dispatch, sideBar, alwaysOnTop }) => {
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
            {sideBar && <Divider orientation="vertical" />}
            <Resizable
                defaultSize={NAV_DEFAULT_SIZE}
                minWidth="150px"
                enable={NAV_RESIZE_DIRECTION}
                style={sideBar ? undefined : NAV_HIDDEN_STYLE}
                as="nav"
            >
                {titleBar && (
                    <div className="nav-titlebar">
                        {alwaysOnTop && <div className="pin-status">📌</div>}
                        <WindowBar />
                    </div>
                )}
                <SideBar headings={headings} path={path} />
            </Resizable>
        </Box>
    );
};
