import AppBar from '@mui/material/AppBar';
import Typography from '@mui/material/Typography';
import Container from '@mui/material/Container';
import logo from '../images/logo.png'
import '../styles/ResponsiveAppBar.css';
import { Link } from 'react-router-dom';

const ResponsiveAppBar = () => {

return (
    <AppBar className="AppBar" position="static" style={{
        backgroundImage : "linear-gradient(#211f1a,black)",
        }}>
        <Container maxWidth={false}>
        <div className="headerContainer">
            <img src={logo} href="/" className="headerLogo" alt="krABMaga" />
            <Typography
                variant="h3"
                noWrap  
                sx={{
                display: { md: 'flex' },
                fontFamily: 'monospace',
                fontWeight: 200,
                letterSpacing: '.3rem',
                color: 'inherit',
                textDecoration: 'none',
                }}
            >
            <Link to="/" style={{textDecoration: 'none', color: 'white'}}>
                krABMaga
            </Link>
            </Typography>
        </div>
        </Container>
    </AppBar>
);
};
export default ResponsiveAppBar;