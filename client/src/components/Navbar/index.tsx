import { Link } from 'react-router-dom';
import style from './Navbar.module.css';

const Navbar = () => (
    <nav className={style.navbar}>
        <ul>
            <li><Link to="/">Home</Link></li>
        </ul>
    </nav>
);

export default Navbar;
