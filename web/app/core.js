document.onload = () => {
    const menu = document.querySelector('#menu');
    const menuIcon = document.querySelector('#menu-icon');
    
    if (menu) {
        menuIcon.addEventListener('mousedown', () => {
            menu.classList.toggle('hidden');
        });
    
        document.addEventListener('click', (event) => {
            if (!menu.contains(event.target) && !menuIcon.contains(event.target)) {
                menu.classList.add('hidden');
            }
        });
    }    
}