import { Link } from 'react-router-dom';
import style from './Header.module.css';

const Header = () => (
  <header className={style.header}>
    <div className={style.inner}>
      <Link to="/" className={style.brand}>
        <img src="/favicon.svg" alt="" className={style.logo} />
        Palantir
      </Link>
    </div>
  </header>
);

export default Header;
