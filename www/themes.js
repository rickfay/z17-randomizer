function init() {
    randomBackgrounds();
    themeToggle();
}

function themeToggle() {
    let darkMode = localStorage.getItem('darkMode');
    const darkModeToggle = document.querySelector('#theme-toggle');

    const enableDarkMode = () => {
        document.body.classList.add('darkMode');
        localStorage.setItem('darkMode', 'enabled');
    }
    const disableDarkMode = () => {
        document.body.classList.remove('darkMode');
        localStorage.setItem('darkMode', null);
    }

    // Defaults to using the user's theme preference, but will remember if they toggle it.
    if (darkMode === 'enabled' || (darkMode === null && window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        enableDarkMode();
    }

    darkModeToggle.addEventListener('click', () => {
        darkMode = localStorage.getItem('darkMode');
        if (darkMode !== 'enabled') {
            enableDarkMode();
        } else {
            disableDarkMode();
        }
    });
}

function randomBackgrounds() {
    let backgrounds= [
        'img/bg.png',
        'img/bg_flippers_dungeon.png',
        'img/bg_throne_room.png',
        'img/bg_ice.png',
        'img/bg_desert.png',
        'img/bg_lorule_sacred_realm.png'
    ];
    let index = Math.floor(Math.random() * backgrounds.length);
    let classToAdd = backgrounds[index];

    document.body.setAttribute("style", `background-image: url("${classToAdd}");`);
}

// Wait for everything to be loaded, and then bootstrap the app
window.addEventListener("load", function bootstrap() {
    init();
});
