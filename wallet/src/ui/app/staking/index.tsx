import { Route } from 'react-router-dom';

import StakeHome from './home';
import StakeNew from './stake';

export const routes = (
    <>
        <Route path="stake" element={<StakeHome />} />
        <Route path="stake/new" element={<StakeNew />} />
    </>
);
