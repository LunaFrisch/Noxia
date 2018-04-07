table! {
    packages (userid) {
        userid -> Int4,
        packagelist -> Text,
    }
}

table! {
    profiles (userid) {
        userid -> Int4,
        name -> Text,
        role -> Text,
        favor -> Int4,
        totalclients -> Int4,
        totalvisits -> Int4,
    }
}

table! {
    purchases (userid) {
        userid -> Int4,
        gelbooru -> Bool,
        rule34 -> Bool,
        e621 -> Bool,
        yandere -> Bool,
        lmgtfy -> Bool,
    }
}

table! {
    roles (rolename) {
        rolename -> Text,
        tip -> Bool,
        clockin -> Bool,
        clockout -> Bool,
        confirm -> Bool,
        deny -> Bool,
        list -> Bool,
        offer -> Bool,
        feedback -> Bool,
        gelbooru -> Bool,
        rule34 -> Bool,
        e621 -> Bool,
        yandere -> Bool,
        urban -> Bool,
        lmgtfy -> Bool,
        checkfavor -> Bool,
        setrole -> Bool,
        setworker -> Bool,
        removeworker -> Bool,
        blacklist -> Bool,
        say -> Bool,
        checkfeedback -> Bool,
        setpackage -> Bool,
        profile -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    packages,
    profiles,
    purchases,
    roles,
);
