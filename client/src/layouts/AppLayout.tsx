import Navbar from '../components/Navbar';
import { Outlet } from 'react-router-dom';

const AppLayout = () => (
    <>
        <Navbar/>
        <Outlet />
    </>
);

export default AppLayout;
