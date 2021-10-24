enum SkinImage {
    ButtonPrev,
    ButtonPlay,
    ButtonPause,
    ButtonStop,
    ButtonNext,
    ButtonEject,
    ButtonPrevPressed,
    ButtonPlayPressed,
    ButtonPausePressed,
    ButtonStopPressed,
    ButtonNextPressed,
    ButtonEjectPressed,
}

fn load_skin_data() {
    vec![
    (ButtonPrev, 0, 0, 22, 18),
    (ButtonPlay, 23, 0, 45, 18),
    (ButtonPause, 46, 0, 68, 18),
    (ButtonStop, 69, 0, 91, 18),
    (ButtonNext, 92, 0, 114, 18),
    (ButtonEject, 115, 0, 137, 18),
    (ButtonPrevPressed, 0, 18, 22, 36),
    (ButtonPlayPressed, 23, 18, 45, 36),
    (ButtonPausePressed, 46, 18, 68, 36),
    (ButtonStopPressed, 69, 18, 91, 36),
    (ButtonNextPressed, 92, 18, 114, 36),
    (ButtonEjectPressed, 115, 18, 137, 36),
    ];
}