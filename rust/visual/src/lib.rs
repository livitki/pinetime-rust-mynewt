/*  -- BEGIN BLOCKS --
<xml xmlns="http://www.w3.org/1999/xhtml">
  <variables>
    <variable type="" id="{0:uoGS8Ut7qbrMxY+DT">sensor</variable>
    <variable type="" id="+}/LZ2zn|5qu,#8oejOB">poll_time</variable>
    <variable type="" id="JrEYqy*^|anEi:~R^~.I">sensor_data</variable>
    <variable type="" id="@k2]dh4RQ4wcej_BOd0w">millisec</variable>
    <variable type="" id="2j?Wbn0?}roI+y7F?5La">network</variable>
    <variable type="" id="pW#H4#F0sFg9vRWVahxG">listener</variable>
    <variable type="" id="GWosHRY^?bqfY=/t_nZ4">host</variable>
    <variable type="" id="NF^UZN;DB?_$s$sOX,z|">port</variable>
    <variable type="" id="rO29tV+a|lgtrwy%}$(R">uri</variable>
    <variable type="" id="}BJ%MDhN{/((:45os0[w">payload</variable>
    <variable type="" id="g(}L4e(EcS=?9mr@Sse?">eventq</variable>
    <variable type="" id="Hj$Va78.9KJj+Lu5K)3^">COAP_HOST</variable>
    <variable type="" id=",2Y@MNx0?i|V5kdPi{7e">SENSOR_DEVICE</variable>
    <variable type="" id="m9:Gp-](=Oa5m}09zui?">COAP_PORT</variable>
    <variable type="" id="D?Y#zZ*zFk8Jm_p[S5DK">SENSOR_POLL_TIME</variable>
    <variable type="" id="?gCb!GjcWt8HUN_k}C(3">COAP_URI</variable>
    <variable type="" id="[x7;!Z38oG?*yUxfdyCs">NETWORK_DEVICE</variable>
    <variable type="" id="RO2d0yDF8:oHMw[-Uj3t">device_id</variable>
  </variables>
  <block type="on_start" id="^vGc^r(:XZYLALbQLyIo" x="-163" y="-537">
    <statement name="STMTS">
      <block type="procedures_callnoreturn" id="Mm2BrF=;%Gz1p}l4Tp^B">
        <mutation name="os::sysinit"></mutation>
        <next>
          <block type="variables_set" id="i+pd{7.}H{kf=7UzaH7j">
            <field name="VAR" id=",2Y@MNx0?i|V5kdPi{7e" variabletype="">SENSOR_DEVICE</field>
            <value name="VALUE">
              <block type="text" id="r}Pwl*3h6;BSjf}6q}_5">
                <field name="TEXT">temp_stm32_0</field>
              </block>
            </value>
            <next>
              <block type="variables_set" id="GKnT!z1`gQ+xv6#!g80W">
                <field name="VAR" id="D?Y#zZ*zFk8Jm_p[S5DK" variabletype="">SENSOR_POLL_TIME</field>
                <value name="VALUE">
                  <block type="math_number" id="3rp#U?EVe|SIbr:4|VC$">
                    <field name="NUM">10000</field>
                  </block>
                </value>
                <next>
                  <block type="variables_set" id="[1rCPWPI#eusp-6$d357">
                    <field name="VAR" id="[x7;!Z38oG?*yUxfdyCs" variabletype="">NETWORK_DEVICE</field>
                    <value name="VALUE">
                      <block type="text" id="MyfH7$!p7x9g(hD#^y;}">
                        <field name="TEXT">bc95g_0</field>
                      </block>
                    </value>
                    <next>
                      <block type="procedures_callnoreturn" id="?45T0G%^uW0zb$AF52MS">
                        <mutation name="start_sensor_listener">
                          <arg name="sensor"></arg>
                          <arg name="poll_time"></arg>
                        </mutation>
                        <value name="ARG0">
                          <block type="variables_get" id="/-~CoQ[o^Q~`H118qV7[">
                            <field name="VAR" id=",2Y@MNx0?i|V5kdPi{7e" variabletype="">SENSOR_DEVICE</field>
                          </block>
                        </value>
                        <value name="ARG1">
                          <block type="variables_get" id="R3|phL$LS.v]02Xa~T2Z">
                            <field name="VAR" id="D?Y#zZ*zFk8Jm_p[S5DK" variabletype="">SENSOR_POLL_TIME</field>
                          </block>
                        </value>
                        <next>
                          <block type="procedures_callnoreturn" id="=L}9h{.d]#2`Ik^C)y(E">
                            <mutation name="network::start_server_transport">
                              <arg name="network"></arg>
                            </mutation>
                            <value name="ARG0">
                              <block type="variables_get" id="~g@^9.ux#-@#;dV_$:Vg">
                                <field name="VAR" id="[x7;!Z38oG?*yUxfdyCs" variabletype="">NETWORK_DEVICE</field>
                              </block>
                            </value>
                            <next>
                              <block type="controls_whileUntil" id="Kg?$VH1V?QD,$qrQ~G;D">
                                <field name="MODE">WHILE</field>
                                <value name="BOOL">
                                  <block type="logic_boolean" id="1[D]X7DI7=It5Mk[P!Bx">
                                    <field name="BOOL">TRUE</field>
                                  </block>
                                </value>
                                <statement name="DO">
                                  <block type="procedures_callnoreturn" id=")N~NC;SGqu{_fH^gjCY-">
                                    <mutation name="os::eventq_run">
                                      <arg name="eventq"></arg>
                                    </mutation>
                                    <value name="ARG0">
                                      <block type="procedures_callreturn" id="fGjx@!+p;F7W[S9_qbL|">
                                        <mutation name="os::eventq_dflt_get"></mutation>
                                      </block>
                                    </value>
                                  </block>
                                </statement>
                              </block>
                            </next>
                          </block>
                        </next>
                      </block>
                    </next>
                  </block>
                </next>
              </block>
            </next>
          </block>
        </next>
      </block>
    </statement>
  </block>
  <block type="procedures_defnoreturn" id="KY`rjgq5Z]H8nKy_%SbR" x="-162" y="-113">
    <mutation>
      <arg name="sensor" varid="{0:uoGS8Ut7qbrMxY+DT"></arg>
      <arg name="poll_time" varid="+}/LZ2zn|5qu,#8oejOB"></arg>
    </mutation>
    <field name="NAME">start_sensor_listener</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
    <statement name="STACK">
      <block type="procedures_callnoreturn" id="7zl-|;/*+_em`,/$a^|n">
        <mutation name="sensor::set_poll_rate_ms">
          <arg name="sensor"></arg>
          <arg name="millisec"></arg>
        </mutation>
        <value name="ARG0">
          <block type="variables_get" id="F;QLh/,eA62MaiFP.g1%">
            <field name="VAR" id="{0:uoGS8Ut7qbrMxY+DT" variabletype="">sensor</field>
          </block>
        </value>
        <value name="ARG1">
          <block type="variables_get" id=".,}J,XVMp1!~,-?zkobg">
            <field name="VAR" id="+}/LZ2zn|5qu,#8oejOB" variabletype="">poll_time</field>
          </block>
        </value>
        <next>
          <block type="procedures_callnoreturn" id="jfH_849uIS#u3r0982fN">
            <mutation name="sensor::register_listener">
              <arg name="sensor"></arg>
              <arg name="listener"></arg>
            </mutation>
            <value name="ARG0">
              <block type="variables_get" id="3V_/(y`hbSCQ3REDNFtN">
                <field name="VAR" id="{0:uoGS8Ut7qbrMxY+DT" variabletype="">sensor</field>
              </block>
            </value>
            <value name="ARG1">
              <block type="text" id=":R(*i7s;@j/qr9e@:n}A">
                <field name="TEXT">handle_sensor_data</field>
              </block>
            </value>
          </block>
        </next>
      </block>
    </statement>
  </block>
  <block type="procedures_defnoreturn" id="t9O!Y}._PlhTjJHyAvwn" x="288" y="-113">
    <mutation>
      <arg name="sensor_data" varid="JrEYqy*^|anEi:~R^~.I"></arg>
    </mutation>
    <field name="NAME">send_sensor_data</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
    <statement name="STACK">
      <block type="variables_set" id="beNuQuu#|f3qT00c}nMD">
        <field name="VAR" id="Hj$Va78.9KJj+Lu5K)3^" variabletype="">COAP_HOST</field>
        <value name="VALUE">
          <block type="text" id="G@jYX?I:`@T+Z~ff(w`A">
            <field name="TEXT">104.199.85.211</field>
          </block>
        </value>
        <next>
          <block type="variables_set" id="LN-QL:c|Ms8+),A;G4NS">
            <field name="VAR" id="m9:Gp-](=Oa5m}09zui?" variabletype="">COAP_PORT</field>
            <value name="VALUE">
              <block type="math_number" id="r-PbMQjR_hRXUe-5|U98">
                <field name="NUM">5683</field>
              </block>
            </value>
            <next>
              <block type="variables_set" id="-]HpZ-$)EM6a+XwPEfCb">
                <field name="VAR" id="?gCb!GjcWt8HUN_k}C(3" variabletype="">COAP_URI</field>
                <value name="VALUE">
                  <block type="text" id="/V4p21=uiYj%7,sZZen{">
                    <field name="TEXT">v2/things/IVRiBCcR6HPp_CcZIFfOZFxz_izni5xc_KO-kgSA2Y8</field>
                  </block>
                </value>
                <next>
                  <block type="procedures_callnoreturn" id="vuZ0E?,5Xzs`hHE|UO_c">
                    <mutation name="network::init_server_post">
                      <arg name="host"></arg>
                      <arg name="port"></arg>
                      <arg name="uri"></arg>
                    </mutation>
                    <value name="ARG0">
                      <block type="variables_get" id="$yzgE}m)5[=9=Xx2oDrS">
                        <field name="VAR" id="Hj$Va78.9KJj+Lu5K)3^" variabletype="">COAP_HOST</field>
                      </block>
                    </value>
                    <value name="ARG1">
                      <block type="variables_get" id=";|Y7[uP`D8kb}CzU}T~e">
                        <field name="VAR" id="m9:Gp-](=Oa5m}09zui?" variabletype="">COAP_PORT</field>
                      </block>
                    </value>
                    <value name="ARG2">
                      <block type="variables_get" id="jsq$LF8;$s3}{uK$3Jnh">
                        <field name="VAR" id="?gCb!GjcWt8HUN_k}C(3" variabletype="">COAP_URI</field>
                      </block>
                    </value>
                    <next>
                      <block type="variables_set" id="Z0$@)|y921DbN0kSddbO">
                        <field name="VAR" id="}BJ%MDhN{/((:45os0[w" variabletype="">payload</field>
                        <value name="VALUE">
                          <block type="text_join" id="ukEB%p?m?y%59Ev6g}c:">
                            <mutation items="2"></mutation>
                            <value name="ADD0">
                              <block type="text_join" id="cBf4X!9;-TWAc{+fY)j1">
                                <mutation items="2"></mutation>
                                <value name="ADD0">
                                  <block type="text" id="AsP-SFsjWozHQ~B@Ccm$">
                                    <field name="TEXT">device</field>
                                  </block>
                                </value>
                                <value name="ADD1">
                                  <block type="variables_get" id="S$Po$6mG*:d;/=zn-`}B">
                                    <field name="VAR" id="RO2d0yDF8:oHMw[-Uj3t" variabletype="">device_id</field>
                                  </block>
                                </value>
                              </block>
                            </value>
                            <value name="ADD1">
                              <block type="variables_get" id="T[Ju=/CeZ[i;2x(txWn5">
                                <field name="VAR" id="JrEYqy*^|anEi:~R^~.I" variabletype="">sensor_data</field>
                              </block>
                            </value>
                          </block>
                        </value>
                        <next>
                          <block type="procedures_callnoreturn" id="tc/EWZNB{if?Hk[r(w8A">
                            <mutation name="network::do_server_post">
                              <arg name="payload"></arg>
                            </mutation>
                            <value name="ARG0">
                              <block type="variables_get" id="GR(jDz*rk4:T6}0[b6hs">
                                <field name="VAR" id="}BJ%MDhN{/((:45os0[w" variabletype="">payload</field>
                              </block>
                            </value>
                          </block>
                        </next>
                      </block>
                    </next>
                  </block>
                </next>
              </block>
            </next>
          </block>
        </next>
      </block>
    </statement>
  </block>
  <block type="procedures_defnoreturn" id="fl`yW^^_4l+SknDsFvl4" x="-162" y="138">
    <mutation>
      <arg name="sensor_data" varid="JrEYqy*^|anEi:~R^~.I"></arg>
    </mutation>
    <field name="NAME">handle_sensor_data</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
    <statement name="STACK">
      <block type="procedures_callnoreturn" id="10ko7!S__/Nj7|w(3E?B">
        <mutation name="send_sensor_data">
          <arg name="sensor_data"></arg>
        </mutation>
        <value name="ARG0">
          <block type="variables_get" id="k=r51M^s3H#o.;((~X@y">
            <field name="VAR" id="JrEYqy*^|anEi:~R^~.I" variabletype="">sensor_data</field>
          </block>
        </value>
      </block>
    </statement>
  </block>
  <block type="procedures_defnoreturn" id="[fJkz[Ay6X2JAYvRINM5" x="37" y="537">
    <mutation>
      <arg name="sensor" varid="{0:uoGS8Ut7qbrMxY+DT"></arg>
      <arg name="millisec" varid="@k2]dh4RQ4wcej_BOd0w"></arg>
    </mutation>
    <field name="NAME">sensor::set_poll_rate_ms</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defnoreturn" id="Ny4W{:ArnXek8XO}zd[o" x="-263" y="562">
    <field name="NAME">os::sysinit</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defnoreturn" id="x8HC)hVxiLkuLgW_uU]i" x="438" y="537">
    <mutation>
      <arg name="network" varid="2j?Wbn0?}roI+y7F?5La"></arg>
    </mutation>
    <field name="NAME">network::start_server_transport</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defnoreturn" id="`{AbNvOKicSC/jqf@x{x" x="37" y="612">
    <mutation>
      <arg name="sensor" varid="{0:uoGS8Ut7qbrMxY+DT"></arg>
      <arg name="listener" varid="pW#H4#F0sFg9vRWVahxG"></arg>
    </mutation>
    <field name="NAME">sensor::register_listener</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defnoreturn" id="Xw{:o4uh]%~h6;PKK!yM" x="-262" y="638">
    <mutation>
      <arg name="eventq" varid="g(}L4e(EcS=?9mr@Sse?"></arg>
    </mutation>
    <field name="NAME">os::eventq_run</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defnoreturn" id="qITn-Ew_Q:4+f^QQU^n," x="437" y="613">
    <mutation>
      <arg name="host" varid="GWosHRY^?bqfY=/t_nZ4"></arg>
      <arg name="port" varid="NF^UZN;DB?_$s$sOX,z|"></arg>
      <arg name="uri" varid="rO29tV+a|lgtrwy%}$(R"></arg>
    </mutation>
    <field name="NAME">network::init_server_post</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defreturn" id="K,gOd#6I?+U_3r}Ki.qh" x="-263" y="712">
    <field name="NAME">os::eventq_dflt_get</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
  <block type="procedures_defnoreturn" id=")Gs9;%U7lg,N1F~dBmh[" x="437" y="687">
    <mutation>
      <arg name="payload" varid="}BJ%MDhN{/((:45os0[w"></arg>
    </mutation>
    <field name="NAME">network::do_server_post</field>
    <comment pinned="false" h="80" w="160">Describe this function...</comment>
  </block>
</xml>
-- END BLOCKS -- */