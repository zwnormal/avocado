use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Jwt {
    access_token_expire_in: i64,
    refresh_token_expire_in: i64,
}

impl Jwt {
    pub(crate) fn access_token_expire_time(&self) -> Result<i64> {
        Ok(Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.access_token_expire_in))
            .ok_or(anyhow!("unable to get access token expire time"))?
            .timestamp())
    }

    pub(crate) fn refresh_token_expire_time(&self) -> Result<i64> {
        Ok(Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.refresh_token_expire_in))
            .ok_or(anyhow!("unable to get refresh token expire time",))?
            .timestamp())
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Rsa {
    private_key: String,
    public_key: String,
}

impl Rsa {
    pub(crate) fn private_key(&self) -> Vec<u8> {
        general_purpose::STANDARD
            .decode(self.private_key.clone())
            .unwrap()
    }

    pub(crate) fn public_key(&self) -> Vec<u8> {
        general_purpose::STANDARD
            .decode(self.public_key.clone())
            .unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) rsa: Rsa,
    pub(crate) jwt: Jwt,
}

impl Config {
    #[cfg(not(any(test, debug_assertions)))]
    pub(crate) fn new() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::new(
                "config.user.yaml",
                config::FileFormat::Yaml,
            ))
            .build()
            .unwrap();
        config.try_deserialize::<Config>().unwrap()
    }

    #[cfg(any(test, debug_assertions))]
    pub(crate) fn new() -> Self {
        let jwt = Jwt {
            access_token_expire_in: 600,
            refresh_token_expire_in: 43200,
        };
        let rsa = Rsa {
           private_key: "LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0tCk1JSUpRZ0lCQURBTkJna3Foa2lHOXcwQkFRRUZBQVNDQ1N3d2dna29BZ0VBQW9JQ0FRQyt1Ym9lVXZFb0lqaVgKMHo1Y3pCMHM2aTdDR2xJR2tHRy82c25VNVA2RGp1RE1heEJDT3Z6QnprYTZwRUYzTGNjMHJrRENOcWpKc0REYwp2YVBmdW1vL0haRldETy9uSm8rYVZXbGZ0ZE0vVjhua3JqakUrYnRLd1pvZDBZbncyclRZWG40L1JLR2MxcWx1ClNiQjltT202NlNBVVcrWlAzRFBvN1dXMENzT2RDNDlPVTJYRTdYQ09jVVd1RWpqRko2OUhyNExIV0lBZlJOUlgKNGFNYzByZncrWThIRVR2Z29VdWpicHMvUk92MnAzLytCZjFRVzNhM3ZsSkpVczJKcGhKbVZ0cms0dnpqYVJzcwpMQWlPRjRLbUhnb1BUWXhJT1BzKzB6bHNGdUhSeHNQUzd0eUNGUGkvaTFEb3krQWM3QWpuaDZvREhoU1JmeUVnCkQ2eVBBczZnVVpWSlpMWDN4RGJsckJhZ1ZrMzZaeiswY3dURk9vMEl1aVg1TG4ydzlYTWt6MGhNbWYyVDJLRWQKenZRY1Noc3l6bklpNFFTdkRreFNWbTVuREJzeFFxRmpNeUh6a0dEOUdabDl6amE0VG0wcTJLTEFURDR4b1UxRQo2YjBWbE5iRys1ckVRNjNvUTlUTFhKSjVRNkJmeGdwb09HOVh1aVZtVmEyM2J5R254V3lqeXVVbFF0QTg4OERECmIyUmdRbEVHNEw5bk9IY0hyZGMxQktlZ1N2Z3M0MG9SclVSV1crUld1UjdhZVBSQ3Q4Szl4R1puUzFpelRsQWIKSEpFalhEVkMyVUlWMnVwZnJoSFhuR0pKNXV0Z3A4aDJLMGk4cDJnVmZJVWhDU2ZFQjl0dXFGWUd2Z3luWE9PeApCa0wyL3JnNFQvbHppcVNTVWIxSkxROU1jVXJhWVFJREFRQUJBb0lDQUF3dEgwekFUdFVaR2pQWE1Ld0wrL0RhCklwSVJ6QnJQTE81cnR1YjRsNmNlZ0F1TTZrVFFMMExRUUlPNTZOUUxBQ1RFMnkySy84dWdpLzRMdldLVExDVkwKWmZQd2I0QjZYY0NrbzAyUTlpT2RIeTZTVU4yQ2RBcVp3WlVWdzVDMDVjYktnWlRZZE5qbWUzcW5DYlY3QmJFQgpJU01ybXRDT1ptSUFoZld3bWp4MGVIbXZxVmVoTVFabkJPUFFGZi9BUU5oNnhBYW1COGYzUlBjM1dJcUdpV2ZCCkJ2aVVpUllXTmhZQkFiaFFWaUF4SVdLK1R5WEtNUklaL0hiaDZtMVhST1Z6dGhYV3gvNWN2M2RhZUZGWi91Vk0KRmFoVWN4aEpTd0QveXpLelkwZFpRVGMrT0tXMVlRd1pnTmpiNnpydGRzUVJUVEZWd2twbnExMjRvbk9lNGtJTgpmcGhib1ZqbkxMQmdFd3VjMkVWZllVNFdVUUhFb0VqRHdpdUhRSTdDZVJPUFlqNWUreG9PbS9zN1J5MXI5Zi9OCkdlZXlSZjlkRmhGVHVDamV4ak50VThObmZ5S2FTYTVBbGRuaUxkR3JhL3FZREFVaVduKzlRdVA5ZzBYUUxZOS8KUUR0ZGw2dGYvVDE5UlR4UStSMjV3ODQ3Smdxbm5TMFZJZ25xS1d1MjEwTHRsbk1jb1Nna01ScjNrcnVvUWQ2TApYcmxlYnFDWDRkaUhYbVNSQ3p5QldxeUphZCtYZlQzeG5VSTIvaWJKaW1KMmErTC9xMWlWMEQ2RlhjT0g0QUwrClI1SmFwcTNWcEpPSUFCeUF5dHVmclhWS0ZueFRuZ0Q0ZUpNRUtadWwydHA5Sy9ObElGOVpZMnBFRlE5ZC9NeEIKeUQycnA0T3E3ZGVRQXN5eUhNeUJBb0lCQVFENzJ0MDBUS1hmN28rVTJ3bUI0MmdVYXEwRzJkcVYyemRERU45ZApMOXRmUkUvWHlxdEwrOUdFWGdXai9kV1hkSDRFeWNJb3M3S2Z1Y2JHYS9vY2lzVzRJTTBqaG83NGFjdlVkdk1LCmlCRWpOSXh0MXJYSWdHT1NuQ1BFc0piN0V4OHVRQ1RsMzJzbVIrd3JWRU5zVFZhOVY5Sk9sTnAxemhQak9DeWYKbGRMK2dDV3pjQVhYNXNlK040RkxVWkdkNGJpQlFjSXQ3NlBQOWpIS3IxTG1UMUp1WEpmNERXVkMrU0RPRlRGKwpVM2JmMVI5akx3Z2g0TE16bytSRWI1MkR4MVg1MlFRM1IzNEdoQnVlSDE0SmF1TDF2SVpsYVRVejRkcEd1aU9MCnU2bXphTGtrcWpqSWxBZUtKQ3hXNXVkbGxrNnk2amd1YVRNMTV6RmE1RzBqSm1FRkFvSUJBUURCM1U2dkZWYysKZnJKUHpiZ2ZFZGJmWlJlWlFXMTY5b0Z1bE9aVnRFSVJEYXJYendXUzd4aGZvK1pKS2FFUGNaeWRBcU9HQ21aMgpFbVhEVUhaTkhNUnBSRlc5TitZNFFSSkxQMFZCZjIxVmRlejZUUEtoSWdWaU81T293OWp4cWpqaWF4clpkZFUrClR0eVhZTjNYK0k5M09pdy92YjZEQVdBQzZSNWd1QVBBWU9HbVRXNkdqU1NiUmNMVmFFVHFGOE1KLzQ2eTZWaXoKSlFhVUdkWCtYcG9Oc21CK2ZWMnQ2bU9DbE8yOGNQWnlpSXBDK0NRMDFBSDl2ZzlYbjhFM3FYZUFTeXZ2Y1duMAp2cHhpMzBmVk9FUkZVS1pnenZ2UjJpUk1OdlBtWU5zNEpsUWJCRjNOR2w2ZE9hcG43NnlKTnRLeDFRUk12WjJjCjk3Y0kyRm5kZUVLdEFvSUJBRy9NaEExSFNRclN0Mkg0cVl2REN0eWxUaEdRZTN1eVNDRzVSNjN5Skl0eG80L0QKSTY4akphcUFvamhkNFliTjYzTHNyUWVzWFVxU0NTN2psNjg3MnZFVmZucWFyTFNSSWk0NmIwRHBqZEgyN2ErRQpkRVdBMkxLb0pTaVd5eTdCR2tKSnJ4ZHJ5Z0RBUkpJbllaYWQ3amMyMk9DUkt6aklOZ1BUVjl1dmdtMy9MTDB1CnFTbGlCS0RudXJsWWlDOXhsNW1wT3kxeEIySzFndnphUHc2RmY2NEY1Ri8wY2xMOGpCZHZTUDB1eDhJc3RUWXUKRm9vVFdQay9jNEQ5bXdwNFhpMytxK1J1RS8rSWZVbi8ycHFMbkJ4Qmg5ZFVrSTFyWUoyczIwS1lGNGpRbi84cQpvUEpJNDkvcWx0UXN3K05RaWJiYVNmNHdMTTlzeFhmM2xvclg2cEVDZ2dFQkFJem1YQm5FV1BTREFQWWZmb0tXClpLVll4QjRPLzVNdk5rTTQxVy9VNDIraGoxbnFqUktOMUJrdTltQ1djUWpZZWd0anI1WHNXU3ZYa3o2eGJFbi8KYm9GVHVUMGRNNllQSlV5clg1cUxFdHZhS2JLQjJwWmlNNmVCeFlBVFY4bk0vaXNSelNIZ0VteUd4SzBCS2p1ZAozQ2U3a0EyRHlzTk1iSXV4QXZNb3l4RTNXTW1WM1k3TlpwMnp4amQwNFNHQ3c2NndudlpKd3IzeFd5YW80RzlRCk1vZGg3ZUFCSGVVWlJkblNoRGRCb3R6N2dBSE1lQVF0VW9VcGJkMHhyeXpWYnBWQURWNnAvMmh5WVJjN1VPb1gKU3dkVjVwYlYyUkJjaTZVWkM0dVZPYjlGbkR0R29TWkExQ05SbGhpVWljSHdwcEc1VUxhMTJhZ01JbXU3VXhVMApJc0VDZ2dFQVVndlo1Z01ldzRLaVMyYUxwSDl0ODFBUE0zdjl3TG1nTHd2NXdpcjBJRWthNVFtc2ZPRzZGZWJ4CkE1dmhWMWpJN05NekMvcXY5S01OMjQzSlp5SFhNUDFjTmZjUzV1Wks3YUR6MGo1WERGTi84T3hSdWRMNVplS1gKQ2YwdWlhSVFudlIxVi80K2JRVlk4RXI3WGhkWkFCUVlxT0xaMjczYVBJdkpzWTVKU29vWEhwQnUzaXcwVlZ5UgpVU1o5dW9LT09UR2VsdXRpZnFycDZpMEpDTlRTN0ZWZzN1NUoyNDZSSDR1VHBDNGV6cUE5V3RwZ3NEYXJ5dFRuClFzbi9NbStaTHpkTFN4TjhOMm5udkl6UVVLcDBjOFlEWEh4Z0JIY2w3M1d6UStXOUhpR0luenpJSzEydStxUkEKNksxTm5VUVRZRFkyempPMS9qeU1ZOXNQOGVqT2l3PT0KLS0tLS1FTkQgUFJJVkFURSBLRVktLS0tLQo=".to_string(),
           public_key: "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQ0lqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FnOEFNSUlDQ2dLQ0FnRUF2cm02SGxMeEtDSTRsOU0rWE13ZApMT291d2hwU0JwQmh2K3JKMU9UK2c0N2d6R3NRUWpyOHdjNUd1cVJCZHkzSE5LNUF3amFveWJBdzNMMmozN3BxClB4MlJWZ3p2NXlhUG1sVnBYN1hUUDFmSjVLNDR4UG03U3NHYUhkR0o4TnEwMkY1K1AwU2huTmFwYmttd2ZaanAKdXVrZ0ZGdm1UOXd6Nk8xbHRBckRuUXVQVGxObHhPMXdqbkZGcmhJNHhTZXZSNitDeDFpQUgwVFVWK0dqSE5LMwo4UG1QQnhFNzRLRkxvMjZiUDBUcjlxZC8vZ1g5VUZ0MnQ3NVNTVkxOaWFZU1psYmE1T0w4NDJrYkxDd0lqaGVDCnBoNEtEMDJNU0RqN1B0TTViQmJoMGNiRDB1N2NnaFQ0djR0UTZNdmdIT3dJNTRlcUF4NFVrWDhoSUErc2p3TE8Kb0ZHVlNXUzE5OFEyNWF3V29GWk4rbWMvdEhNRXhUcU5DTG9sK1M1OXNQVnpKTTlJVEpuOWs5aWhIYzcwSEVvYgpNczV5SXVFRXJ3NU1VbFp1Wnd3Yk1VS2hZek1oODVCZy9SbVpmYzQydUU1dEt0aWl3RXcrTWFGTlJPbTlGWlRXCnh2dWF4RU90NkVQVXkxeVNlVU9nWDhZS2FEaHZWN29sWmxXdHQyOGhwOFZzbzhybEpVTFFQUFBBdzI5a1lFSlIKQnVDL1p6aDNCNjNYTlFTbm9FcjRMT05LRWExRVZsdmtWcmtlMm5qMFFyZkN2Y1JtWjB0WXMwNVFHeHlSSTF3MQpRdGxDRmRycVg2NFIxNXhpU2VicllLZklkaXRJdktkb0ZYeUZJUWtueEFmYmJxaFdCcjRNcDF6anNRWkM5djY0Ck9FLzVjNHFra2xHOVNTMFBUSEZLMm1FQ0F3RUFBUT09Ci0tLS0tRU5EIFBVQkxJQyBLRVktLS0tLQo=".to_string()
       };
        Config { rsa, jwt }
    }
}
