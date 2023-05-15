import { Link } from "react-router-dom"

export const Navbar = () => (
    <nav>
        <ul>
            <li><Link to="/">Home</Link></li>
            <li><Link to="/settings">Settings</Link></li>
        </ul>
    </nav>
)
