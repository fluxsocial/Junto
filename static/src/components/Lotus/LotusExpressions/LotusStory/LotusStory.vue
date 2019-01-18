<template>
    <div class="lotusStory">
        <input type="text" class="lotusStory__title" placeholder="Title (optional)" maxlength="255">
        <div class="lotusStory__editor">
            <froala :tag="'textarea'" :config="config"></froala>
        </div>
    </div>
</template>

<script>

    export default {
        data() {
            return {
                config: {
                    charCounterCount: false, 
                    width: '100%',
                    // heightMax: '60vh',
                    // height: '100%',                    
                    // colorsText: ['#49D8E9', '#FF8EAE', '#5FF9CF', 'REMOVE'],
                    toolbarButtons: ['fullscreen', 'bold', 'italic', 'insertLink', '|', 'paragraphFormat', 'formatUL', 'formatOL', 'insert'],
                    paragraphFormat: {N: 'Normal', H1: 'Heading 1'},
                    lineHeights: {Default: '2'},
                    zIndex: 1,
                    theme: 'junto',
                    // colorsBackground: [],
                    // colorsHEXInput: false,
                    // height: '64vh',
                    // zIndex: 2000, 
                    // tooltips: false,
                    // autofocus: true,
                    tableEditButtons: [''],
                    placeholderText: '',
                    imageDefaultWidth: '100%',
                    imageEditButtons: [],
                    imageResize: false,
                    pastePlain: true, 
                    videoEditButtons: [],
                    videoResize: false,
                    videoInsertButtons: [],
                    videoAllowedTypes: [],
                    videoMove: false,
                    videoUpload: false,
                    linkAlwaysBlank: true,
                    linkEditButtons: [],
                    linkText: false,
                    linkInsertButtons: [],
                    quickInsertButtons: ['image', 'video', 'hr'],
                    toolbarVisibleWithoutSelection: false, 
                    toolbarInline: true,
                    // fontFamily: {
                    //     "Raleway": "Raleway",
                    // },
                    toolbarVisibleWithoutSelection: false, 
                    // iconsTemplate: 'material_design'
                },
            }
        }, 

        created() {
                $.FroalaEditor.DefineIcon('insert', {NAME: 'plus'});
                $.FroalaEditor.RegisterCommand('insert', {
                title: 'Insert HTML',
                focus: true,
                undo: true,
                refreshAfterCallback: true,
                callback: function () {

                    this.html.insert(`<h2></h2>`);
                    }
                });

        }

    }

</script>

<style lang="scss">
    .lotusStory__editor {
        font-size: 1.7rem;
        line-height: 1.5;
        font-family: Avenir;
    }   

    .lotusStory__editor .fr-box .fr-wrapper .fr-element {
        padding: 2rem 0;
    }

    .lotusStory__editor .fr-box .fr-wrapper .fr-element p {
        margin-bottom: 2rem;
    }

    .lotusStory__editor .fr-box .fr-wrapper .fr-element h1 {
        font-size: 4rem;
    }

    .lotusStory__editor .fr-box .fr-wrapper .fr-element hr {
        width: 90%;
        margin-left: 50%;
        transform: translateX(-50%);
        border: 1px solid #eee;
        margin-top: 4rem;
        margin-bottom: 4rem;
        // display: block;
        text-align: center;
    }

    .lotusStory__editor .fr-box .fr-wrapper .fr-element ul {
        margin-left: 6rem;
        margin-bottom: 2rem;
    }

    .lotusStory__editor .fr-box .fr-wrapper .fr-element ol {
        margin-left: 6rem;
        margin-bottom: 2rem;
    }

    // .juntoTE .fr-box .fr-wrapper .fr-element p .fr-video iframe {
    //     width: 100%;
    //     height: 400px;
    // }

    iframe {
        width: 100%;
        height: 400px; 
    }

</style>
