import { Outlet } from 'react-router-dom';
import Header from '../components/Header';
import style from './AppLayout.module.css';

const AppLayout = () => (
  <>
    <Header />
    <main className={style.main}>
      <Outlet />
    </main>
  </>
);

export default AppLayout;
