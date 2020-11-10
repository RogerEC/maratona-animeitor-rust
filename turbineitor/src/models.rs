
// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, answernumber)]
#[table_name = "answertable"]
pub struct Answertable {
    pub contestnumber: i32,
    pub answernumber: i32,
    pub runanswer: String,
    pub yes: bool,
    pub fake: bool,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, sitenumber, bkpnumber)]
#[table_name = "bkptable"]
pub struct Bkptable {
    pub contestnumber: i32,
    pub sitenumber: i32,
    pub bkpnumber: i32,
    pub usernumber: i32,
    pub bkpdate: i32,
    pub bkpfilename: String,
    pub bkpdata: u32,
    pub bkpstatus: String,
    pub bkpsize: i32,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, clarsitenumber, clarnumber)]
#[table_name = "clartable"]
pub struct Clartable {
    pub contestnumber: i32,
    pub clarsitenumber: i32,
    pub clarnumber: i32,
    pub usernumber: i32,
    pub clardate: i32,
    pub clardatediff: i32,
    pub clardatediffans: i32,
    pub clarproblem: i32,
    pub clardata: String,
    pub claranswer: Option<String>,
    pub clarstatus: String,
    pub clarjudge: Option<i32>,
    pub clarjudgesite: Option<i32>,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber)]
#[table_name = "contesttable"]
pub struct Contesttable {
    pub contestnumber: i32,
    pub contestname: String,
    pub conteststartdate: i32,
    pub contestduration: i32,
    pub contestlastmileanswer: Option<i32>,
    pub contestlastmilescore: Option<i32>,
    pub contestlocalsite: i32,
    pub contestpenalty: i32,
    pub contestmaxfilesize: i32,
    pub contestactive: bool,
    pub contestmainsite: i32,
    pub contestkeys: String,
    pub contestunlockkey: String,
    pub contestmainsiteurl: String,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, langnumber)]
#[table_name = "langtable"]
pub struct Langtable {
    pub contestnumber: i32,
    pub langnumber: i32,
    pub langname: String,
    pub langextension: String,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(lognumber)]
#[table_name = "logtable"]
pub struct Logtable {
    pub lognumber: i32,
    pub contestnumber: i32,
    pub sitenumber: i32,
    pub loguser: Option<i32>,
    pub logip: String,
    pub logdate: i32,
    pub logtype: String,
    pub logdata: String,
    pub logstatus: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, problemnumber)]
#[table_name = "problemtable"]
pub struct Problemtable {
    pub contestnumber: i32,
    pub problemnumber: i32,
    pub problemname: String,
    pub problemfullname: Option<String>,
    pub problembasefilename: Option<String>,
    pub probleminputfilename: Option<String>,
    pub probleminputfile: Option<u32>,
    pub probleminputfilehash: Option<String>,
    pub fake: bool,
    pub problemcolorname: Option<String>,
    pub problemcolor: Option<String>,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, runsitenumber, runnumber)]
#[table_name = "runtable"]
pub struct Runtable {
    pub contestnumber: i32,
    pub runsitenumber: i32,
    pub runnumber: i32,
    pub usernumber: i32,
    pub rundate: i32,
    pub rundatediff: i32,
    pub rundatediffans: i32,
    pub runproblem: i32,
    pub runfilename: String,
    pub rundata: u32,
    pub runanswer: i32,
    pub runstatus: String,
    pub runjudge: Option<i32>,
    pub runjudgesite: Option<i32>,
    pub runanswer1: i32,
    pub runjudge1: Option<i32>,
    pub runjudgesite1: Option<i32>,
    pub runanswer2: i32,
    pub runjudge2: Option<i32>,
    pub runjudgesite2: Option<i32>,
    pub runlangnumber: i32,
    pub autoip: Option<String>,
    pub autobegindate: Option<i32>,
    pub autoenddate: Option<i32>,
    pub autoanswer: Option<String>,
    pub autostdout: Option<u32>,
    pub autostderr: Option<u32>,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, sitenumber)]
#[table_name = "sitetable"]
pub struct Sitetable {
    pub contestnumber: i32,
    pub sitenumber: i32,
    pub siteip: String,
    pub sitename: String,
    pub siteactive: bool,
    pub sitepermitlogins: bool,
    pub sitelastmileanswer: Option<i32>,
    pub sitelastmilescore: Option<i32>,
    pub siteduration: Option<i32>,
    pub siteautoend: Option<bool>,
    pub sitejudging: Option<String>,
    pub sitetasking: Option<String>,
    pub siteglobalscore: String,
    pub sitescorelevel: i32,
    pub sitenextuser: i32,
    pub sitenextclar: i32,
    pub sitenextrun: i32,
    pub sitenexttask: i32,
    pub sitemaxtask: i32,
    pub updatetime: i32,
    pub sitechiefname: String,
    pub siteautojudge: Option<bool>,
    pub sitemaxruntime: i32,
    pub sitemaxjudgewaittime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, sitenumber, sitestartdate)]
#[table_name = "sitetimetable"]
pub struct Sitetimetable {
    pub contestnumber: i32,
    pub sitenumber: i32,
    pub sitestartdate: i32,
    pub siteenddate: i32,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, sitenumber, tasknumber)]
#[table_name = "tasktable"]
pub struct Tasktable {
    pub contestnumber: i32,
    pub sitenumber: i32,
    pub usernumber: i32,
    pub tasknumber: i32,
    pub taskstaffnumber: Option<i32>,
    pub taskstaffsite: Option<i32>,
    pub taskdate: i32,
    pub taskdatediff: i32,
    pub taskdatediffans: i32,
    pub taskdesc: Option<String>,
    pub taskfilename: Option<String>,
    pub taskdata: Option<u32>,
    pub tasksystem: bool,
    pub taskstatus: String,
    pub colorname: Option<String>,
    pub color: Option<String>,
    pub updatetime: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(contestnumber, usersitenumber, usernumber)]
#[table_name = "usertable"]
pub struct Usertable {
    pub contestnumber: i32,
    pub usersitenumber: i32,
    pub usernumber: i32,
    pub username: String,
    pub userfullname: String,
    pub userdesc: Option<String>,
    pub usertype: String,
    pub userenabled: bool,
    pub usermultilogin: bool,
    pub userpassword: Option<String>,
    pub userip: Option<String>,
    pub userlastlogin: Option<i32>,
    pub usersession: Option<String>,
    pub usersessionextra: Option<String>,
    pub userlastlogout: Option<i32>,
    pub userpermitip: Option<String>,
    pub userinfo: Option<String>,
    pub updatetime: i32,
    pub usericpcid: Option<String>,
}
