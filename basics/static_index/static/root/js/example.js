jQuery(document).ready(function () {
    let rotation = 0;
    jQuery("img").click(function () {
        rotation += 360;
        jQuery("img").css({'transform': 'rotate(' + rotation + 'deg)'});
    });
});
